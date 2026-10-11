//! Dinomight Knight, the True Dracofighter
use super::prelude::*;

pub const ID: u32 = 58984738;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRUE_DRACO_KING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with s/t
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_ADD_EXTRA_TRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_EXTRA_TRIBUTE);
    // e1:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e1, LOCATION_SZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsType,TYPE_CONTINUOUS))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_CONTINUOUS))));
    // e1:SetValue(POS_FACEUP)
    effect::set_value(d, e1, POS_FACEUP as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // search
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned() and rp~=tp
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h) && a.rp != a.tp
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TRUE_DRACO_KING) and c:IsContinuousTrap()
    //     and (c:IsAbleToHand() or (c:GetActivateEffect():IsActivatable(tp,true,true) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0))
    card::is_set_card(d, c, &[SET_TRUE_DRACO_KING as u16])
        && card::is_continuous_trap(d, c)
        && (card::is_able_to_hand(d, c, None) || {
            let te = card::get_activate_effect(d, c).first().copied().nil();
            effect::is_activatable(d, te, tp, true, true) && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        })
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local tc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // aux.ToHandOrElse(tc,tp,function(c) ... end, function(c) ... end, aux.Stringid(id,2))
    if let Some(tc) = tc {
        let check = |d: &mut Duel, _c: CardId| {
            // local te=tc:GetActivateEffect()
            // return te:IsActivatable(tp,true,true) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
            let te = card::get_activate_effect(d, tc).first().copied().nil();
            effect::is_activatable(d, te, tp, true, true) && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        };
        let eg = a.eg.clone().map_or(Value::Nil, Value::Group);
        let (ep, ev, re, r, rp) = (a.ep, a.ev, a.re, a.r, a.rp);
        let oper = |d: &mut Duel, _g: &CardSet| -> i32 {
            // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
            // local te=tc:GetActivateEffect()
            let te = card::get_activate_effect(d, tc).first().copied().nil();
            // local tep=tc:GetControler()
            let tep = card::get_controler(d, tc);
            // local cost=te:GetCost()
            // if cost then cost(te,tep,eg,ep,ev,re,r,rp,1) end
            if let Some(cost) = effect::get_cost(d, te) {
                let args = [
                    Value::Effect(te),
                    Value::Int(tep as i64),
                    eg.clone(),
                    Value::Int(ep as i64),
                    Value::Int(ev as i64),
                    Value::effect(re),
                    Value::Int(r as i64),
                    Value::Int(rp as i64),
                    Value::Int(1),
                ];
                cost.call(d, &args);
            }
            0
        };
        aux::to_hand_or_else(d, tc, tp, Some(&check), Some(&oper), Some(aux::stringid(ID, 2)));
    }
}
