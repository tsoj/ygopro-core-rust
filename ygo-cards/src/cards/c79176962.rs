//! Fantastic Striborg
//! 旋風機ストリボーグ
use super::prelude::*;

pub const ID: u32 = 79176962;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Monsters Tributed for the face-up Tribute Summon of this card are returned to the hand instead of going to the GY
    // local e0a=Effect.CreateEffect(c)
    let e0a = effect::create_effect(d, c);
    // e0a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0a, EFFECT_TYPE_SINGLE);
    // e0a:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0a, EFFECT_MATERIAL_CHECK);
    // e0a:SetValue(s.valcheck)
    effect::set_value_fn(d, e0a, lua_fn(valcheck));
    // c:RegisterEffect(e0a)
    card::register_effect(d, c, e0a, false);
    // local e0b=Effect.CreateEffect(c)
    let e0b = effect::create_effect(d, c);
    // e0b:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0b, EFFECT_TYPE_SINGLE);
    // e0b:SetCode(EFFECT_SUMMON_COST)
    effect::set_code(d, e0b, EFFECT_SUMMON_COST);
    // e0b:SetOperation(function() e0a:SetLabel(1) end)
    effect::set_operation(d, e0b, Some(operation(move |d, _a| effect::set_label(d, e0a, &[1]))));
    // c:RegisterEffect(e0b)
    card::register_effect(d, c, e0b, false);
    // Return all cards your opponent controls in this card's column to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.Discard())
    effect::set_cost(d, e1, Some(cost::discard(None, false, None, None, None)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    // if e:GetLabel()==0 then return end
    if effect::get_label(d, e) == 0 {
        return Value::Nil;
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    let c = a.c(1);
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // for mc in mg:Iter() do
    for mc in group::iter(d, &mg) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_HAND)
        effect::set_value(d, e1, LOCATION_HAND as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // mc:RegisterEffect(e1)
        card::register_effect(d, mc, e1, false);
    }
    Value::Nil
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsAbleToHand()
    card::is_controler(d, c, 1 - tp) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetColumnGroup():IsExists(s.thfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let g = card::get_column_group(d, h, 0, 0);
        return group::is_exists(d, &g, &|d, c| thfilter(d, c, a.tp), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local colg=c:GetColumnGroup():Match(Card.IsControler,nil,1-tp)
    let mut colg = card::get_column_group(d, c, 0, 0);
    group::match_(d, &mut colg, &|d, x| card::is_controler(d, x, 1 - a.tp), NO_EX);
    // if c:IsControler(1-tp) then colg:AddCard(c) end
    if card::is_controler(d, c, 1 - a.tp) {
        group::add_card(&mut colg, &CardSet::from_card(c));
    }
    // if #colg>0 then
    if !colg.is_empty() {
        // Duel.SendtoHand(colg,nil,REASON_EFFECT)
        duel::send_to_hand(d, colg, None, REASON_EFFECT, None);
    }
}
