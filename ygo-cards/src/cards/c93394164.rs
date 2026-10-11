//! Parallel Panzer
use super::prelude::*;

pub const ID: u32 = 93394164;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsType,TYPE_LINK))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_type(d, x, TYPE_LINK))), None, None, None, None, None, None);
    // move to a zone it point
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.mtarget)
    effect::set_target(d, e1, Some(cost_target(mtarget)));
    // e1:SetOperation(s.moperation)
    effect::set_operation(d, e1, Some(operation(moperation)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.dcost)
    effect::set_cost(d, e2, Some(cost_target(dcost)));
    // e2:SetTarget(s.dtarget)
    effect::set_target(d, e2, Some(cost_target(dtarget)));
    // e2:SetOperation(s.doperation)
    effect::set_operation(d, e2, Some(operation(doperation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.mtarget(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mtarget(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if chk==0 then
    if a.chk == Some(0) {
        // if not ec then return false end
        let Some(ec) = ec else {
            return false;
        };
        // local p=ec:GetControler()
        let p = card::get_controler(d, ec);
        // local zone=ec:GetLinkedZone()&ZONES_MMZ
        let zone = card::get_linked_zone(d, ec, None) & ZONES_MMZ;
        // return Duel.GetLocationCount(p,LOCATION_MZONE,p,LOCATION_REASON_CONTROL,zone)>0
        return duel::get_location_count(d, p, LOCATION_MZONE as u8, Some(p), Some(LOCATION_REASON_CONTROL), Some(zone)).0 > 0;
    }
    true
}
/// `s.moperation(e,tp,eg,ep,ev,re,r,rp)`
fn moperation(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    // if not ec then return end
    let Some(ec) = card::get_equip_target(d, c) else {
        return;
    };
    // local p=ec:GetControler()
    let p = card::get_controler(d, ec);
    // local zone=ec:GetLinkedZone()&ZONES_MMZ
    let zone = card::get_linked_zone(d, ec, None) & ZONES_MMZ;
    // if Duel.GetLocationCount(p,LOCATION_MZONE,p,LOCATION_REASON_CONTROL,zone)>0 then
    if duel::get_location_count(d, p, LOCATION_MZONE as u8, Some(p), Some(LOCATION_REASON_CONTROL), Some(zone)).0 > 0 {
        // local i=0
        // if not ec:IsControler(tp) then i=16 end
        let i: u32 = if card::is_controler(d, ec, tp) { 0 } else { 16 };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
        // local nseq=math.log(Duel.SelectDisableField(tp,1,LOCATION_MZONE,LOCATION_MZONE,~(zone<<i)),2) - i
        let sel = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, LOCATION_MZONE, Some(!(zone << i)), false);
        let nseq = (sel as f64).log2() - i as f64;
        // Duel.MoveSequence(ec,nseq)
        duel::move_sequence(d, ec, nseq as u8, None);
    }
}
/// `s.dcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if chk==0 then return ec and c:IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        return ec.is_some() && card::is_able_to_grave_as_cost(d, c);
    }
    // Duel.SetTargetCard(ec)
    duel::set_target_card(d, ec.nil());
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.filter(c,g,tc)`
fn filter(c: CardId, g: &CardSet, tc: CardId) -> bool {
    // return c==tc or g:IsContains(c)
    c == tc || group::is_contains(g, c)
}
/// `s.dtarget(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dtarget(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    // if not ec then return end
    let Some(ec) = card::get_equip_target(d, c) else {
        return false;
    };
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c,ec:GetColumnGroup(),ec)
    let col = card::get_column_group(d, ec, 0, 0);
    let g = duel::get_matching_group(d, Some(&|_d, x| filter(x, &col, ec)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.doperation(e,tp,eg,ep,ev,re,r,rp)`
fn doperation(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = tc else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dg=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil,tc:GetColumnGroup(),tc)
    let col = card::get_column_group(d, tc, 0, 0);
    let dg = duel::select_matching_card(d, tp, Some(&|_d, x| filter(x, &col, tc)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
