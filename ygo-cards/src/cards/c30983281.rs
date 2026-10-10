//! Accel Synchro Stardust Dragon
use super::prelude::*;

pub const ID: u32 = 30983281;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_STARDUST_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Special Summon 1 Level 2 or lower Tuner from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e1, 1, ID, 1, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_synchro_summoned(d, c)
    })));
    // e1:SetTarget(s.gysptg)
    effect::set_target(d, e1, Some(cost_target(gysptg)));
    // e1:SetOperation(s.gyspop)
    effect::set_operation(d, e1, Some(operation(gyspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Stardust Dragon" from your Extra Deck (this is treated as a Synchro Summon), then, immediately after this effect resolves, Synchro Summon using monsters you control as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e2, 1, ID, 2, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetCost(Cost.Replaceable(Cost.AND(Cost.SelfTribute,s.exspcost),s.extracon))
    effect::set_cost(
        d,
        e2,
        Some(cost::replaceable(cost::and(vec![cost::self_tribute(), cost_target(exspcost)]), Some(lua_fn(extracon)))),
    );
    // e2:SetTarget(s.exsptg)
    effect::set_target(d, e2, Some(cost_target(exsptg)));
    // e2:SetOperation(s.exspop)
    effect::set_operation(d, e2, Some(operation(exspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.gyspfilter(c,e,tp)`
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(2) and c:IsType(TYPE_TUNER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 2) && card::is_type(d, c, TYPE_TUNER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.gysptg(e,tp,...,chk)`
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.gyspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| gyspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.gyspop(e,tp,...)`
fn gyspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| gyspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.exspfilter(c,e,tp,rc,step)`
fn exspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: Option<CardId>, step: bool) -> bool {
    // local mg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanBeSynchroMaterial),tp,LOCATION_MZONE,0,rc)
    let mg = duel::get_matching_group(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_can_be_synchro_material(d, c, None, None, None))),
        tp,
        LOCATION_MZONE,
        0,
        rc.into(),
    );
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,c,nil,REASON_SYNCHRO)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, Some(c), None, REASON_SYNCHRO);
    // return c:IsCode(CARD_STARDUST_DRAGON) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,rc,c)>0 and (step or (#pg<=0 and aux.CheckSummonGate(tp,2)
    //     and Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,c,nil,mg+c)))
    if !(card::is_code(d, c, &[CARD_STARDUST_DRAGON])
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None))
    {
        return false;
    }
    if duel::get_location_count_from_ex(d, tp, Some(tp), rc.into(), duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    if step {
        return true;
    }
    if !pg.is_empty() || !aux::check_summon_gate(d, tp, 2) {
        return false;
    }
    let mut mgc = mg.clone();
    group::add_card(&mut mgc, &CardSet::from_card(c));
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_synchro_summonable(d, x, None, Some(mgc.clone()), 0, 0)), tp, LOCATION_EXTRA, 0, 1, c.into())
}
/// `s.exspcost(e,tp,...,chk)`
fn exspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.exspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let rc = effect::get_handler(d, a.e);
        return duel::is_existing_matching_card(d, Some(&|d, c| exspfilter(d, c, a.e, a.tp, rc, false)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    true
}
/// `s.exsptg(e,tp,...,chk)`
fn exsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonCount(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_count(d, a.tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.exspop(e,tp,...)`
fn exspop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local stardustc=Duel.SelectMatchingCard(tp,s.exspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,nil,true):GetFirst()
    let sel = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, c| exspfilter(d, c, a.e, a.tp, None, true)),
        a.tp,
        LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if not stardustc then return end
    let Some(stardustc) = group::get_first(d, &sel) else {
        return;
    };
    // stardustc:SetMaterial(nil)
    card::set_material(d, stardustc, CardSet::new());
    // if Duel.SpecialSummon(stardustc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, stardustc, SUMMON_TYPE_SYNCHRO, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // stardustc:CompleteProcedure()
        card::complete_procedure(d, stardustc);
        // local sg=Group.CreateGroup(stardustc)
        let mut sg = CardSet::from_card(stardustc);
        // if Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil,nil) then
        if duel::is_existing_matching_card(d, Some(&|d, x| card::is_synchro_summonable(d, x, None, None, 0, 0)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sync=Duel.SelectMatchingCard(tp,Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,1,nil,nil):GetFirst()
            let sel = duel::select_matching_card(
                d,
                a.tp,
                Some(&|d, x| card::is_synchro_summonable(d, x, None, None, 0, 0)),
                a.tp,
                LOCATION_EXTRA,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            // if sync then
            if let Some(sync) = group::get_first(d, &sel) {
                // sg:AddCard(sync)
                group::add_card(&mut sg, &CardSet::from_card(sync));
                // Duel.SynchroSummon(tp,sync,nil)
                duel::synchro_summon(d, a.tp, sync, None, None, 0, 0);
            }
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // --For the rest of this turn, the monsters Synchro Summoned by this effect are unaffected by your opponent's activated effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e1, aux::stringid(ID, 2));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_IMMUNE_EFFECT)
            effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
            // e1:SetValue(function(e,te) return te:GetOwnerPlayer()==1-e:GetHandlerPlayer() and te:IsActivated() end)
            effect::set_value_fn(d, e1, lua_fn(immune_value));
            // e1:SetReset(RESETS_STANDARD_PHASE_END&~RESET_TOFIELD)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END & !RESET_TOFIELD, 1);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
        }
    }
}
/// `function(e,te) return te:GetOwnerPlayer()==1-e:GetHandlerPlayer() and te:IsActivated() end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, e) && effect::is_activated(d, te)
}
/// `s.extracon(base,e,tp,eg,ep,ev,re,r,rp)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let tp = a.p(2);
    // return Duel.IsExistingMatchingCard(s.exspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
    duel::is_existing_matching_card(d, Some(&|d, c| exspfilter(d, c, e, tp, None, false)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
