//! Gaming Gamer GG
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 15778492;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 Machine monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_MACHINE),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_MACHINE, sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // If this card is Special Summoned: You can change all monsters your opponent controls to Attack Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your opponent has a monster in their field or GY: You can detach 1 material from this card; send 1 Machine
    // monster from your Deck/Extra Deck to the GY, then you can apply this effect: ● Choose 1 Machine Xyz Monster in
    // your GY, and this card's name becomes that monster's name until the End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(Card.IsMonster,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsExistingMatchingCard(Card.IsMonster,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
}
/// `aux.AND(Card.IsDefensePos,Card.IsCanChangePosition)`
fn defense_changeable(d: &mut Duel, c: CardId) -> bool {
    card::is_defense_pos(d, c) && card::is_can_change_position(d, c)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsDefensePos,Card.IsCanChangePosition),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&defense_changeable), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,tp,POS_FACEUP_ATTACK)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, a.tp, POS_FACEUP_ATTACK as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsDefensePos,Card.IsCanChangePosition),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&defense_changeable), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then Duel.ChangePosition(g,POS_FACEUP_ATTACK) end
    if !g.is_empty() {
        duel::change_position(d, g, POS_FACEUP_ATTACK as u8, None, None, None, false);
    }
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_MACHINE as u64) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.namefilter(c,current_name)`
fn namefilter(d: &mut Duel, c: CardId, current_name: u32) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsXyzMonster() and not c:IsCode(current_name)
    card::is_race(d, c, RACE_MACHINE as u64) && card::is_xyz_monster(d, c) && !card::is_code(d, c, &[current_name])
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local code=c:GetCode()
    let code = card::get_code(d, c)[0];
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE) and c:IsRelateToEffect(e) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.namefilter,tp,LOCATION_GRAVE,0,1,nil,code)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    let Some(sc) = sc else { return };
    if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) > 0
        && card::is_location(d, sc, LOCATION_GRAVE)
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| namefilter(d, x, code)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // local xyz=Duel.SelectMatchingCard(tp,s.namefilter,tp,LOCATION_GRAVE,0,1,1,nil,code):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, x| namefilter(d, x, code)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if not xyz then return end
        let Some(xyz) = group::get_first(d, &g) else { return };
        // Duel.HintSelection(xyz)
        duel::hint_selection(d, xyz, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // This card's name becomes that monster's name until the End Phase
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_CODE)
        effect::set_code(d, e1, EFFECT_CHANGE_CODE);
        // e1:SetValue(xyz:GetCode())
        let xyz_code = card::get_code(d, xyz)[0] as i64;
        effect::set_value(d, e1, xyz_code);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Reset the card's name manually during the End Phase
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(aux.Stringid(id,4))
        effect::set_description(d, e2, aux::stringid(ID, 4));
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetOperation(function(e)
        //     e1:Reset()
        //     Duel.HintSelection(c)
        //     Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
        // end)
        effect::set_operation(
            d,
            e2,
            Some(operation(move |d, ea| {
                effect::reset(d, e1);
                duel::hint_selection(d, c, true);
                let desc = effect::get_description(d, ea.e);
                duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
            })),
        );
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
