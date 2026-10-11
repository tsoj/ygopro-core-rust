//! Kai-Den Kendo Spirit
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 71614230;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Spirit.AddProcedure(c,EVENT_SUMMON_SUCCESS,EVENT_FLIP)
    spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    // Return this card from the Pendulum Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send to GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    // (sic: the Lua script sets the description on e1 here, not on e2)
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonType,1,nil,SUMMON_TYPE_PENDULUM)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_PENDULUM]), 1, NO_EX)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SendtoHand(c,nil,REASON_EFFECT)
    duel::send_to_hand(d, c, None, REASON_EFFECT, None);
}
fn gyfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetColumnGroup():IsExists(Card.IsControler,1,nil,1-tp)
    let g = card::get_column_group(d, c, 0, 0);
    group::is_exists(d, &g, &|d, x| card::is_controler(d, x, 1 - tp), 1, NO_EX)
}
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_PZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| gyfilter(d, c, a.tp)), a.tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,0,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local pg=Duel.GetMatchingGroup(s.gyfilter,tp,LOCATION_PZONE,0,nil,tp)
    let pg = duel::get_matching_group(d, Some(&|d, c| gyfilter(d, c, a.tp)), a.tp, LOCATION_PZONE, 0, NO_EX);
    // if #pg==0 then return end
    if pg.len() == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELECT as u64);
    // local pc=pg:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &pg, a.tp, 1, 1, false, NO_EX).nil();
    // if not pc then return end
    let Some(pc) = group::get_first(d, &sel) else {
        return;
    };
    // Duel.HintSelection(pc,true)
    duel::hint_selection(d, pc, true);
    // local g=pc:GetColumnGroup():Filter(Card.IsControler,nil,1-tp)
    let cg = card::get_column_group(d, pc, 0, 0);
    let g = group::filter(d, &cg, &|d, x| card::is_controler(d, x, 1 - a.tp), NO_EX);
    // if #g>0 then
    if g.len() > 0 {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
