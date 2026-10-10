//! Cross-Sheep
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 50277355;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Link Summon procedure: 2 monsters with different names
    // Link.AddProcedure(c,nil,2,2,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(lcheck)), None, None);
    // --If a monster is Special Summoned to a zone this card points to: You can apply the following effects, in sequence, based on the card types of the monster(s) this card points to:
    // --● Ritual: Draw 2 cards, then discard 2 cards
    // --● Fusion: Special Summon 1 Level 4 or lower monster from your GY
    // --● Synchro: All monsters you control gain 700 ATK
    // --● Xyz: All monsters your opponent controls lose 700 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_HANDES+CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(aux.zptcon(nil))
    effect::set_condition(d, e1, Some(aux::zptcon(None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // return g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)
    group::check_different_property(d, g, &|d, c| card::get_code(d, c).into_iter().map(i64::from).collect())
}
/// `s.ritfilter` = `aux.FaceupFilter(Card.IsRitualMonster)`
fn ritfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_ritual_monster(d, c)
}
/// `s.fusfilter` = `aux.FaceupFilter(Card.IsType,TYPE_FUSION)`
fn fusfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.synfilter` = `aux.FaceupFilter(Card.IsType,TYPE_SYNCHRO)`
fn synfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.xyzfilter` = `aux.FaceupFilter(Card.IsType,TYPE_XYZ)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.rittg(e,tp,eg,ep,ev,re,r,rp,chk)`: `chk` is the Lua `chk` (0 or 1), passed explicitly by the callers
fn rittg(d: &mut Duel, a: &EffArgs, chk: u8) -> bool {
    if chk == 0 {
        // return e:GetHandler():GetLinkedGroup():IsExists(s.ritfilter,1,nil) and Duel.IsPlayerCanDraw(tp,2)
        let h = effect::get_handler(d, a.e).nil();
        let lg = card::get_linked_group(d, h);
        return group::is_exists(d, &lg, &|d, c| ritfilter(d, c), 1, NO_EX) && duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 2);
    true
}
/// `s.fustg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn fustg(d: &mut Duel, a: &EffArgs, chk: u8) -> bool {
    if chk == 0 {
        // return e:GetHandler():GetLinkedGroup():IsExists(s.fusfilter,1,nil)
        //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
        let h = effect::get_handler(d, a.e).nil();
        let lg = card::get_linked_group(d, h);
        return group::is_exists(d, &lg, &|d, c| fusfilter(d, c), 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.syntg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn syntg(d: &mut Duel, a: &EffArgs, chk: u8) -> bool {
    if chk == 0 {
        // return e:GetHandler():GetLinkedGroup():IsExists(s.synfilter,1,nil)
        //   and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil)
        let h = effect::get_handler(d, a.e).nil();
        let lg = card::get_linked_group(d, h);
        return group::is_exists(d, &lg, &|d, c| synfilter(d, c), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn xyztg(d: &mut Duel, a: &EffArgs, chk: u8) -> bool {
    if chk == 0 {
        // return e:GetHandler():GetLinkedGroup():IsExists(s.xyzfilter,1,nil)
        //   and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
        let h = effect::get_handler(d, a.e).nil();
        let lg = card::get_linked_group(d, h);
        return group::is_exists(d, &lg, &|d, c| xyzfilter(d, c), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return s.rittg(...,0) or s.fustg(...,0) or s.syntg(...,0) or s.xyztg(...,0) end
    if a.chk == Some(0) {
        return rittg(d, a, 0) || fustg(d, a, 0) || syntg(d, a, 0) || xyztg(d, a, 0);
    }
    // if s.rittg(...,0) then s.rittg(...,1) end
    if rittg(d, a, 0) {
        rittg(d, a, 1);
    }
    // if s.fustg(...,0) then s.fustg(...,1) end
    if fustg(d, a, 0) {
        fustg(d, a, 1);
    }
    // if s.syntg(...,0) then s.syntg(...,1) end
    if syntg(d, a, 0) {
        syntg(d, a, 1);
    }
    // if s.xyztg(...,0) then s.xyztg(...,1) end
    if xyztg(d, a, 0) {
        xyztg(d, a, 1);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,2)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 2);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,1,tp,2)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 1, a.tp, 2);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.ritop(e,tp,eg,ep,ev,re,r,rp)`
fn ritop(d: &mut Duel, a: &EffArgs) {
    // if Duel.Draw(tp,2,REASON_EFFECT)==2 then
    if duel::draw(d, a.tp, 2, REASON_EFFECT) == 2 {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,2,2,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, a.tp, None, 2, 2, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
/// `s.fusop(e,tp,eg,ep,ev,re,r,rp)`
fn fusop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.synop(e,tp,eg,ep,ev,re,r,rp)`
fn synop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in g:Iter() do tc:UpdateAttack(700,nil,c) end
    for tc in group::iter(d, &g) {
        card::update_attack(d, tc, 700, None, Some(c), None);
    }
}
/// `s.xyzop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // for tc in g:Iter() do tc:UpdateAttack(-700,nil,c) end
    for tc in group::iter(d, &g) {
        card::update_attack(d, tc, -700, None, Some(c), None);
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if s.rittg(...,0) then s.ritop(...) end
    if rittg(d, a, 0) {
        ritop(d, a);
    }
    // if s.fustg(...,0) then Duel.BreakEffect() s.fusop(...) end
    if fustg(d, a, 0) {
        duel::break_effect(d);
        fusop(d, a);
    }
    // if s.syntg(...,0) then Duel.BreakEffect() s.synop(...) end
    if syntg(d, a, 0) {
        duel::break_effect(d);
        synop(d, a);
    }
    // if s.xyztg(...,0) then Duel.BreakEffect() s.xyzop(...) end
    if xyztg(d, a, 0) {
        duel::break_effect(d);
        xyzop(d, a);
    }
}
