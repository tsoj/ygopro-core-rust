//! Neo Galaxy-Eyes Photon Dragon - Photon Howling
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 28331069;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PHOTON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Xyz Summon procedure: 3 Level 8 monsters OR 1 Rank 8 Xyz Monster
    // Xyz.AddProcedure(c,nil,8,3,s.altmatfilter,aux.Stringid(id,0),3,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(8),
        3,
        Some(xyz::alter_filter(altmatfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(3)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // --Take 1 "Photon" monster from your Deck, and either Special Summon it in Defense Position or attach it to this card as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_xyz_summoned(d, c)
    })));
    // e1:SetTarget(s.spattachtg)
    effect::set_target(d, e1, Some(cost_target(spattachtg)));
    // e1:SetOperation(s.spattachop)
    effect::set_operation(d, e1, Some(operation(spattachop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Tribute 1 other Xyz Monster, and if you do, negate the effects of all other face-up cards currently on the field until the end of this turn.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(Cost.DetachFromSelf(3))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(3), None, None)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.altmatfilter(c,tp,xyzc)`
fn altmatfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsRank(8) and c:IsFaceup()
    card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp)) && card::is_rank(d, c, &[8]) && card::is_faceup(d, c)
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.int(2) == 0 {
        return duel::get_flag_effect(d, tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    // return true
    true
}
/// `s.spcheck(c,e,tp,mmz_chk)`
fn spcheck(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.attachcheck(c,e,tp)`
fn attachcheck(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeXyzMaterial(e:GetHandler(),tp,REASON_EFFECT)
    let h = effect::get_handler(d, e).nil();
    card::is_can_be_xyz_material(d, c, Some(h), Some(tp), Some(REASON_EFFECT))
}
/// `s.spattachfilter(c,e,tp,mmz_chk)`
fn spattachfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return c:IsSetCard(SET_PHOTON) and c:IsMonster() and (s.spcheck(c,e,tp,mmz_chk) or s.attachcheck(c,e,tp))
    card::is_set_card(d, c, &[SET_PHOTON as u16]) && card::is_monster(d, c) && (spcheck(d, c, e, tp, mmz_chk) || attachcheck(d, c, e, tp))
}
/// `s.spattachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spattachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spattachfilter,tp,LOCATION_DECK,0,1,nil,e,tp,mmz_chk) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| spattachfilter(d, x, e, tp, mmz_chk)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spattachop(e,tp,eg,ep,ev,re,r,rp)`
fn spattachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
    // local sc=Duel.SelectMatchingCard(tp,s.spattachfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mmz_chk):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| spattachfilter(d, x, e, tp, mmz_chk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // local b1=s.spcheck(sc,e,tp,mmz_chk)
    let b1 = spcheck(d, sc, e, tp, mmz_chk);
    // local b2=s.attachcheck(sc,e,tp) and c:IsRelateToEffect(e)
    let b2 = attachcheck(d, sc, e, tp) && card::is_relate_to_effect(d, c, e);
    // local op=Duel.SelectEffect(tp,
    //   {b1,aux.Stringid(id,4)}, --"Special Summon it in Defense Position"
    //   {b2,aux.Stringid(id,5)}) --"Attach it to this card as material"
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 4)), (b2, aux::stringid(ID, 5))]);
    // if op==1 then
    if op == Some(1) {
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.Overlay(c,sc,true)
        duel::overlay(d, c, sc, true);
    }
}
/// `s.xyzfilter(c,self_exc)`
fn xyzfilter(d: &mut Duel, c: CardId, self_exc: Option<CardId>) -> bool {
    // return c:IsXyzMonster() and c:IsReleasableByEffect()
    //   and Duel.IsExistingMatchingCard(Card.IsNegatable,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,Group.FromCards(c,self_exc))
    if !(card::is_xyz_monster(d, c) && card::is_releasable_by_effect(d, c, None, None)) {
        return false;
    }
    let mut ex = CardSet::from_card(c);
    if let Some(s) = self_exc {
        group::add_card(&mut ex, &CardSet::from_card(s));
    }
    duel::is_existing_matching_card(d, Some(&card::is_negatable), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into())
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_MZONE,0,1,c,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, Some(c))), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,0,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
    let g = duel::get_matching_group(d, Some(&card::is_negatable), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,#g-1,tp,0)
    let n = (g.len() as i64 - 1) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), n, tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, e) { Some(c) } else { None };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,exc,exc)
    let ex: Exclude = match exc {
        Some(x) => x.into(),
        None => NO_EX,
    };
    let sc = duel::select_matching_card(d, tp, Some(&|d, x| xyzfilter(d, x, exc)), tp, LOCATION_MZONE, 0, 1, 1, false, ex);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, sc.clone(), true);
    // if Duel.Release(sc,REASON_EFFECT)>0 then
    if duel::release(d, sc, REASON_EFFECT, None) > 0 {
        // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,exc)
        let ex: Exclude = match exc {
            Some(x) => x.into(),
            None => NO_EX,
        };
        let g = duel::get_matching_group(d, Some(&card::is_negatable), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ex);
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // tc:NegateEffects(c,RESET_PHASE|PHASE_END)
            card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
        }
    }
}
