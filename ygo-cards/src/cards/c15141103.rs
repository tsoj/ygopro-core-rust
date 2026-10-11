//! Dark Tuner Catastrogue
use super::prelude::*;

pub const ID: u32 = 15141103;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARK_TUNER as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can reveal this card in your hand; immediately after this effect resolves, Normal Summon 1 "Dark Tuner" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.nstg)
    effect::set_target(d, e1, Some(cost_target(nstg)));
    // e1:SetOperation(s.nsop)
    effect::set_operation(d, e1, Some(operation(nsop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, if you control this Normal Summoned/Set card: You can Tribute both this face-up card and 1 face-up non-Tuner with a lower Level, and if you do, Special Summon 1 Synchro Monster from your Extra Deck with a Level equal to the difference in Levels of those monsters (this is treated as a Synchro Summon), then you can destroy 1 card your opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsNormalSummoned() end)
    effect::set_condition(d, e2, Some(condition(normal_summoned_cond)));
    // e2:SetTarget(s.darksynchtg)
    effect::set_target(d, e2, Some(cost_target(darksynchtg)));
    // e2:SetOperation(s.darksynchop)
    effect::set_operation(d, e2, Some(operation(darksynchop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.nsfilter(c)
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARK_TUNER) and c:IsSummonable(true,nil)
    card::is_set_card(d, c, &[SET_DARK_TUNER as u16]) && card::is_summonable(d, c, true, None, 0, None)
}
// function s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
// function s.nsop(e,tp,eg,ep,ev,re,r,rp)
fn nsop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // Duel.Summon(tp,sc,true,nil)
        duel::summon(d, tp, sc, true, None, 0, None);
    }
}
// function s.nontunerfilter(c,e,tp,lv,mc)
fn nontunerfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64, mc: CardId) -> bool {
    // return not c:IsType(TYPE_TUNER) and c:IsReleasable() and c:IsFaceup() and c:HasLevel() and c:IsLevelBelow(lv-1)
    //     and Duel.IsExistingMatchingCard(s.darksyncfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,lv-c:GetLevel(),Group.FromCards(c,mc))
    if card::is_type(d, c, TYPE_TUNER)
        || !card::is_releasable(d, c, None, None)
        || !card::is_faceup(d, c)
        || !card::has_level(d, c)
        || !card::is_level_below(d, c, (lv - 1) as u32)
    {
        return false;
    }
    let lv2 = lv - card::get_level(d, c) as i64;
    duel::is_existing_matching_card(
            d,
            Some(&|d, x| darksyncfilter(d, x, e, tp, lv2, Some(from_cards(c, mc)))),
            tp,
            LOCATION_EXTRA,
            0,
            1,
            NO_EX,
        )
}
/// `Group.FromCards(a, b)`
fn from_cards(a: CardId, b: CardId) -> CardSet {
    let mut g = CardSet::from_card(a);
    group::add_card(&mut g, &CardSet::from_card(b));
    g
}
// function s.darksyncfilter(c,e,tp,lv,mg)
fn darksyncfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64, mg: Option<CardSet>) -> bool {
    // return c:IsSynchroMonster() and c:IsLevel(lv) and Duel.GetLocationCountFromEx(tp,tp,mg,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    let ex: Exclude = match &mg {
        Some(g) => g.into(),
        None => NO_EX,
    };
    card::is_synchro_monster(d, c)
        && card::is_level(d, c, &[lv as u32])
        && duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
// function s.darksynchtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn darksynchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, None, None, REASON_SYNCHRO);
        // return #pg<=0 and c:IsReleasable() and Duel.IsExistingMatchingCard(s.nontunerfilter,tp,LOCATION_MZONE,0,1,c,e,tp,c:GetLevel(),c)
        let lv = card::get_level(d, c) as i64;
        return pg.is_empty()
            && card::is_releasable(d, c, None, None)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| nontunerfilter(d, x, e, tp, lv, c)),
                tp,
                LOCATION_MZONE,
                0,
                1,
                c.into(),
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,c,2,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(CardSet::from_card(c)), 2, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.darksynchop(e,tp,eg,ep,ev,re,r,rp)
fn darksynchop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, None, None, REASON_SYNCHRO);
    // if #pg>0 then return end
    if !pg.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) {
        // local lv=c:GetLevel()
        let mut lv = card::get_level(d, c) as i64;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.nontunerfilter,tp,LOCATION_MZONE,0,1,1,c,e,tp,lv,c):GetFirst()
        let sel = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| nontunerfilter(d, x, e, tp, lv, c)),
            tp,
            LOCATION_MZONE,
            0,
            1,
            1,
            false,
            c.into(),
        )
        .nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &sel) else {
            return;
        };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // lv=lv-sc:GetLevel()
        lv -= card::get_level(d, sc) as i64;
        // if Duel.Release(Group.FromCards(c,sc),REASON_EFFECT)==2 then
        if duel::release(d, from_cards(c, sc), REASON_EFFECT, None) == 2 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sync=Duel.SelectMatchingCard(tp,s.darksyncfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,lv):GetFirst()
            let sel = duel::select_matching_card(
                d,
                tp,
                Some(&|d, x| darksyncfilter(d, x, e, tp, lv, None)),
                tp,
                LOCATION_EXTRA,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            let sync = group::get_first(d, &sel);
            // if sync and Duel.SpecialSummon(sync,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
            if let Some(sync) = sync {
                if duel::special_summon(d, sync, SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                    // sync:CompleteProcedure()
                    card::complete_procedure(d, sync);
                    // if Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
                    if duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
                        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
                        // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
                        let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
                        // if #g>0 then
                        if !g.is_empty() {
                            // Duel.HintSelection(g)
                            duel::hint_selection(d, g.clone(), true);
                            // Duel.BreakEffect()
                            duel::break_effect(d);
                            // Duel.Destroy(g,REASON_EFFECT)
                            duel::destroy(d, g, REASON_EFFECT, None, None);
                        }
                    }
                }
            }
        }
    }
}
// function(e) return e:GetHandler():IsNormalSummoned() end
fn normal_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_normal_summoned(d, c)
}
