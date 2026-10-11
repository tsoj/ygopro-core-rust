//! Dark Tuner Nightmare Hand
//! ＤＴナイトメア・ハンド
use super::prelude::*;

pub const ID: u32 = 51546708;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DARK_TUNER],
    ..ScriptInfo::DEFAULT
};

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
    // If this card is Normal Summoned: You can Special Summon 1 Level 2 or lower monster from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn, if you control this Normal Summoned/Set card: You can Tribute both this face-up card and 1 face-up non-Tuner with a lower Level, and if you do, Special Summon 1 Synchro Monster from your Extra Deck with a Level equal to the difference in Levels of those monsters (this is treated as a Synchro Summon)
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e) return e:GetHandler():IsNormalSummoned() end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetTarget(s.darksynchtg)
    effect::set_target(d, e3, Some(cost_target(darksynchtg)));
    // e3:SetOperation(s.darksynchop)
    effect::set_operation(d, e3, Some(operation(darksynchop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARK_TUNER) and c:IsSummonable(true,nil)
    card::is_set_card(d, c, &[SET_DARK_TUNER as u16]) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)`
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
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp)`
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
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(2) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 2) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `level <= n` for a Lua number `n` (a negative `n` is never reached)
fn level_below(d: &mut Duel, c: CardId, n: i64) -> bool {
    n >= 0 && card::is_level_below(d, c, n as u32)
}
/// `s.nontunerfilter(c,e,tp,lv,mc)`
fn nontunerfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64, mc: CardId) -> bool {
    // return not c:IsType(TYPE_TUNER) and c:IsReleasable() and c:IsFaceup() and c:HasLevel() and c:IsLevelBelow(lv-1)
    //     and Duel.IsExistingMatchingCard(s.darksyncfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,lv-c:GetLevel(),Group.FromCards(c,mc))
    if card::is_type(d, c, TYPE_TUNER)
        || !card::is_releasable(d, c, None, None)
        || !card::is_faceup(d, c)
        || !card::has_level(d, c)
        || !level_below(d, c, lv - 1)
    {
        return false;
    }
    let cl = card::get_level(d, c) as i64;
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(mc));
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| darksyncfilter(d, x, e, tp, lv - cl, Some(&g))),
        tp,
        LOCATION_EXTRA,
        0,
        1,
        NO_EX,
    )
}
/// `s.darksyncfilter(c,e,tp,lv,mg)`
fn darksyncfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64, mg: Option<&CardSet>) -> bool {
    // return c:IsSynchroMonster() and c:IsLevel(lv) and Duel.GetLocationCountFromEx(tp,tp,mg,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    if !card::is_synchro_monster(d, c) {
        return false;
    }
    if lv < 0 || !card::is_level(d, c, &[lv as u32]) {
        return false;
    }
    let count = match mg {
        Some(g) => duel::get_location_count_from_ex(d, tp, Some(tp), g.into(), duel::ExtraCard::Card(c), None).0,
        None => duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0,
    };
    count > 0 && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
/// `function(e) return e:GetHandler():IsNormalSummoned() end`
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_normal_summoned(d, h)
}
/// `s.darksynchtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn darksynchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, None, None, REASON_SYNCHRO);
        // return #pg<=0 and c:IsReleasable() and Duel.IsExistingMatchingCard(s.nontunerfilter,tp,LOCATION_MZONE,0,1,c,e,tp,c:GetLevel(),c)
        let cl = card::get_level(d, c) as i64;
        return pg.is_empty()
            && card::is_releasable(d, c, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| nontunerfilter(d, x, e, tp, cl, c)), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,c,2,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(CardSet::from_card(c)), 2, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.darksynchop(e,tp,eg,ep,ev,re,r,rp)`
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
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| nontunerfilter(d, x, e, tp, lv, c)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
        let sc = group::get_first(d, &sel);
        // if not sc then return end
        let Some(sc) = sc else { return };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // lv=lv-sc:GetLevel()
        lv -= card::get_level(d, sc) as i64;
        // if Duel.Release(Group.FromCards(c,sc),REASON_EFFECT)==2 then
        let mut rel = CardSet::from_card(c);
        group::add_card(&mut rel, &CardSet::from_card(sc));
        if duel::release(d, rel, REASON_EFFECT, None) == 2 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sync=Duel.SelectMatchingCard(tp,s.darksyncfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,lv):GetFirst()
            let sel = duel::select_matching_card(d, tp, Some(&|d, x| darksyncfilter(d, x, e, tp, lv, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            let sync = group::get_first(d, &sel);
            // if sync and Duel.SpecialSummon(sync,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
            if let Some(sync) = sync {
                if duel::special_summon(d, sync, SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                    // sync:CompleteProcedure()
                    card::complete_procedure(d, sync);
                }
            }
        }
    }
}
